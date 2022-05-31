import { EventEmitter } from 'events';

export class EventBus {

	static #instance: EventBus;

	private constructor () {
		this.bus = new EventEmitter();
	}

	public bus: EventEmitter;
	
	static get getInstance (): EventBus {
		this.#instance = EventBus.#instance ? EventBus.#instance : new EventBus();
		return this.#instance;
	}

}