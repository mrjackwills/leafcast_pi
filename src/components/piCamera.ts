import { CAMERA_ROTATION, LOCATION_IMAGES } from '../config/env';
import { log } from '../lib/log';
import { promises as fs } from 'fs';
import { Rotation, StillCamera } from 'pi-camera-connect';
import { TPhotoLocation, TRotation } from '../types';
import sharp from 'sharp';

const wrap = <T> () => function (_target: PiCamera, _propertyKey: string, descriptor: PropertyDescriptor): void {
	const original = descriptor.value;
	descriptor.value = async function (t: T): Promise<void> {
		try {
			const result = await original.call(this, t);
			return result;
		} catch (e) {
			log.error(e);
		}
	};
};

class PiCamera {

	#byteLengthCompressed = 0;
	#byteLengthOriginal = 0;
	#camera!: StillCamera;
	#photoTimestamp?: Date;
	#rotation!: Rotation;
	#webp_base64 = '';

	constructor (rotation: TRotation) {
		this.#rotation = rotation === '0' ? Rotation.Rotate0 : rotation === '90' ? Rotation.Rotate90 : rotation === '180' ? Rotation.Rotate180 : Rotation.Rotate270;
		this.#camera = new StillCamera({
			rotation: this.#rotation
		});
	}

	get photo (): string {
		return this.#webp_base64;
	}
	
	get rotation (): Rotation {
		return this.#rotation;
	}

	get size_compressed (): number {
		return this.#byteLengthCompressed;
	}

	get size_original (): number {
		return this.#byteLengthOriginal;
	}
	
	get timestamp (): Date {
		return this.#photoTimestamp ?? new Date();
	}

	async #convertImage (imageBuffer: Buffer): Promise<void> {
		try {
			const image_sharp = sharp(imageBuffer);
			const sharpBuffer = await image_sharp.resize({ width: 600, fit: 'inside' }).webp({ quality: 85 }).toBuffer();
			this.#byteLengthCompressed = sharpBuffer.byteLength;
			this.#webp_base64 = sharpBuffer.toString('base64');
		} catch (e) {
			log.error(e);
		}
	}

	#getDirectoryFileName (): TPhotoLocation {
		const data = this.timestamp;
		const filename = `${this.#zeroPad(data.getHours())}-${this.#zeroPad(data.getMinutes())}-${this.#zeroPad(data.getSeconds())}`;
		const directory =`${data.getFullYear()}-${this.#zeroPad(data.getMonth() + 1)}-${this.#zeroPad(data.getDate())}`;
		return { directory, filename };
	}

	async #directoryExists (directory: string): Promise<boolean> {
		try {
			await fs.access(`${LOCATION_IMAGES}/${directory}`);
			return true;
		} catch (e) {
			return false;
		}
	}

	async #createDirectory (directory: string): Promise<void> {
		await fs.mkdir(`${LOCATION_IMAGES}/${directory}`);
	}
	
	async #saveToDisk (imageBuffer: Buffer): Promise<void> {
		const { directory, filename } = this.#getDirectoryFileName();
		const directoryExists = await this.#directoryExists(directory);
		if (!directoryExists) await this.#createDirectory(directory);
		// Only save if bigger than 2.5MB, i.e. is daytime
		if (this.#byteLengthOriginal < 2500000) return;
		await fs.writeFile(`${LOCATION_IMAGES}/${directory}/${directory}_${filename}.jpg`, imageBuffer);
	}

	async #takePhoto (): Promise<Buffer> {
		const timestamp = new Date();
		const imageBuffer = await this.#camera.takeImage();
		this.#photoTimestamp = timestamp;
		this.#byteLengthOriginal = imageBuffer.byteLength;
		return imageBuffer;
	}

	#zeroPad (unit: number): string {
		return String(unit).padStart(2, '0');
	}

	@wrap<boolean>()
	async newPhoto (save: boolean): Promise<void> {
		const imageBuffer = await this.#takePhoto();
		if (save) await this.#saveToDisk(imageBuffer);
		await this.#convertImage(imageBuffer);
	}
}

export type TPiCamera = PiCamera
export const piCamera = new PiCamera(CAMERA_ROTATION);