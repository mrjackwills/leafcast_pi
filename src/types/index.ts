import { Rotation } from 'pi-camera-connect';

type recievedData = { message: 'photo' |'force-update', unique?: string }

export type TCustomEmitter = {
	data?: string;
	emiterName: 'wsMessage' | 'wsOpen'
}

export type TLogLevels = 'debug' | 'error' | 'verbose' | 'warn'

export type TLoggerColors = { readonly [ index in TLogLevels ]: string };

export type TPhoto = { message: 'photo', data: { image?: string, timestamp: Date, imageSize_compressed?: number, imageSize_original?: number, piInfo: TPiStatus, rotation: Rotation } }

export type TPiStatus = { [ K in 'internalIp' | 'piVersion' | 'totalFileSize'] : string } & { [ K in 'uptime' | 'nodeUptime' | 'numberImages'] : number }

export type TRotation = '0' | '90' | '180' | '270'

export type TWSfromServer = {
	data?: recievedData
	error?: { message: string, code: number }
	unique?: string
}

export type TWSToSever = { data: TPhoto, cache?: boolean, unique?:string }

export type TPhotoLocation = {directory: string, filename: string}