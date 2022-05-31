import { api_version } from '../config/api_version';
import { exec } from 'child_process';
import { LOCATION_IMAGES, LOCATION_IP_ADDRESS } from '../config/env';
import { uptime } from 'os';
import { promises as fs } from 'fs';
import { TPiStatus } from '../types';

const directoryExists = async (directory: string): Promise<boolean> => {
	try {
		await fs.stat(directory);
		return true;
	} catch (e) {
		return false;
	}
};

export const getIp = async (): Promise<string> => {
	try {
		const ip_address = await fs.readFile(LOCATION_IP_ADDRESS, 'utf-8');
		return ip_address.trim();
	} catch (e) {
		return '[null]';
	}
};

const du_directory = async (): Promise<string> => new Promise((resolve, reject) =>{
	exec(`du -sb .`, { cwd: LOCATION_IMAGES }, (err, stdout) => {
		if (err) reject(err);
		const humanReadable = stdout.split('\t')[0];
		resolve(humanReadable ?? '0');
	});
});

const numberImages = async (): Promise<number> => new Promise((resolve, reject) =>{
	exec(`find . -type f | wc -l`, { cwd: LOCATION_IMAGES }, (err, stdout) => {
		if (err) reject(err);
		const humanReadable = Number(stdout.split('\t')[0]?.trim());
		resolve(!isNaN(humanReadable)? humanReadable : 0);
	});
});

const totalFileSize = async (): Promise<string> => {
	const exists = await directoryExists(LOCATION_IMAGES);
	if (!exists) return 'directory error';
	const output = await du_directory();
	return output;
};

export const getPiInfo = async (): Promise<TPiStatus> => {
	const output = {
		internalIp: await getIp(),
		piVersion: api_version,
		nodeUptime: Math.trunc(process.uptime()),
		uptime: uptime(),
		numberImages: await numberImages(),
		totalFileSize: await totalFileSize(),
		// connected_since:
	};
	return output;
};