import { getPiInfo } from './components/piInfo';
import { log } from './lib/log';
import { parseMessage } from './lib/messageParser';
import { piCamera, TPiCamera } from './components/piCamera';
import { reconnectingWebSocket, TReconnectingWebSocket } from './lib/websocket';
import { schedule } from 'node-cron';
import { TPhoto } from './types';

class Handler {

	#piCamera!: TPiCamera;
	#ws!: TReconnectingWebSocket;

	constructor (ws: TReconnectingWebSocket, piCamera: TPiCamera) {
		this.#ws = ws;
		this.#piCamera = piCamera;
		this.#startSchedules();
		this.#openListeners();
	}

	#openListeners (): void {

		this.#ws.bus.on('wsOpen', async () => {
			log.debug('websocket connection opened');
		});

		this.#ws.bus.on('wsMessage', async (data: string) => {
			// this should be wrapped in a try catch?
			const message = parseMessage(data);
			log.debug(message);
			if (!message) return;
			if (!message.data?.message) return;
			switch (message.data.message) {
			case 'force-update': {
				await this.#piCamera.newPhoto(false);
				await this.#sendPhoto(message.unique);
				break;
			}
			case 'photo': {
				await this.#sendPhoto(message.unique);
				break;
			}
			}
		});
	}

	async #sendPhoto (unique?: string): Promise<void> {
		const piInfo = await getPiInfo();
		const data: TPhoto = {
			message: <const>'photo',
			data: {
				image: this.#piCamera.photo,
				timestamp: this.#piCamera.timestamp,
				imageSize_compressed: this.#piCamera.size_compressed,
				imageSize_original: this.#piCamera.size_original,
				rotation: this.#piCamera.rotation,
				piInfo
			}
		};
		this.#ws.sendToServer({ data, cache: true, unique });
	}

	async #startSchedules () :Promise<void> {
		await this.#piCamera.newPhoto(false);
		schedule('*/5 * * * *', () => this.#piCamera.newPhoto(true));
	}
	
}

new Handler(reconnectingWebSocket, piCamera);
