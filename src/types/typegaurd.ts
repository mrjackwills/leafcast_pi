import { TRotation } from './index';

export const isRotation = (x: string) : x is TRotation => {
	const allowedRotation = [ '0', '90', '180', '270' ];
	return allowedRotation.includes(x);
};
