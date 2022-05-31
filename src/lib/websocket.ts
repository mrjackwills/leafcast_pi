import { EventBus } from './events.js';
import { log } from './log';
import { TCustomEmitter, TWSToSever } from '../types';
import { WS_ADDRESS, WS_APIKEY, WS_AUTH_ADDRESS, WS_PASSWORD } from '../config/env';
import Axios from 'axios';
import ws from 'ws';

class ReconnectingWebSocket {

	#connected_since = 0;
	#ping_timeout?: NodeJS.Timeout;
	#reconnection_interval!: number;
	#reconnection_attempts!: number;
	readonly #ws_address: string;
	readonly #ws_auth_address: string;
	readonly #ws_api_key: string;
	readonly #ws_password: string;
	#ws?: ws;
	
	constructor () {
		this.#ws_address = WS_ADDRESS;
		this.#ws_auth_address = WS_AUTH_ADDRESS;
		this.#ws_password = WS_PASSWORD;
		this.#ws_api_key = WS_APIKEY;
		this.#resetReconnectionDetails();
		this.#open();
	}

	#resetReconnectionDetails (): void {
		this.#reconnection_interval = 15000;
		this.#reconnection_attempts = 0;
	}

	#checkServer (): void {
		this.#clear();
		this.#ping_timeout = setTimeout(() => this.#ws?.terminate(), 45 * 1000);
	}

	#clear (): void {
		this.#connected_since = 0;
		if (this.#ping_timeout) clearTimeout(this.#ping_timeout);
	}

	#customEmitter (data: TCustomEmitter): void {
		this.bus.emit(data.emiterName, data.data);
	}

	get ws_connected_at (): number {
		return this.#connected_since;
	}
	
	async #getAccessToken (): Promise<string|void> {
		try {
			const accessToken = await Axios.post(this.#ws_auth_address, { key: this.#ws_api_key, password: this.#ws_password });
			if (!accessToken?.data?.response) throw Error('getTmpAuthKey: !accessCode');
			return accessToken?.data?.response;
		} catch (e) {
			log.error(e);
		}
	}
	
	async #open (): Promise<void> {
		try {
			const accessToken = await this.#getAccessToken();
			if (!accessToken) return this.#reconnect();

			this.#ws = new ws(`${this.#ws_address}/${accessToken}`, [ this.#ws_api_key ]);

			this.#ws.on('open', () => {
				this.#connected_since = Date.now();
				this.#customEmitter({ emiterName: 'wsOpen' });
				this.#resetReconnectionDetails();
			});

			this.#ws.on('ping', () => this.#checkServer()),
			
			this.#ws.on('message', (data: Buffer, isBinary: boolean) => {
				if (!isBinary) this.#customEmitter({ data: data.toString(), emiterName: 'wsMessage' });
			}),

			this.#ws.on('close', (code: number, data: Buffer|undefined) => {
				if (code !== 1000) this.#reconnect();
				this.#clear();
				const reason = data?.toString();
				const error = reason ? reason : `disconnected @ ${new Date}`;
				log.error(error);
			});

			this.#ws.on('error', (code: number, data: Buffer|undefined) => {
				const reason = data?.toString();
				if (reason === 'ECONNREFUSED') this.#reconnect();
				const error = reason ? reason : `Error code: ${code} @ ${new Date}`;
				log.error(error);
			});

		} catch (e) {
			log.error(e);
		}
	}

	#reconnect (): void {
		this.#clear();
		if (this.#reconnection_attempts === 40) this.#reconnection_interval = 1000 * 60 * 5;
		this.#reconnection_attempts ++;
		this.#ws?.removeAllListeners();
		setTimeout(() => this.#open(), this.#reconnection_interval);
	}
	
	public bus = EventBus.getInstance.bus;
	
	async sendToServer (message: TWSToSever): Promise<void> {
		this.#ws?.send(JSON.stringify(message));
	}
}

export type TReconnectingWebSocket = ReconnectingWebSocket
export const reconnectingWebSocket = new ReconnectingWebSocket();