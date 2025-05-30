// @ts-nocheck


// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async getEventNotification() : Promise<boolean> {
    return await TAURI_INVOKE("plugin:notification|get_event_notification");
},
async setEventNotification(enabled: boolean) : Promise<null> {
    return await TAURI_INVOKE("plugin:notification|set_event_notification", { enabled });
},
async getDetectNotification() : Promise<boolean> {
    return await TAURI_INVOKE("plugin:notification|get_detect_notification");
},
async setDetectNotification(enabled: boolean) : Promise<null> {
    return await TAURI_INVOKE("plugin:notification|set_detect_notification", { enabled });
},
async openNotificationSettings() : Promise<null> {
    return await TAURI_INVOKE("plugin:notification|open_notification_settings");
},
async requestNotificationPermission() : Promise<null> {
    return await TAURI_INVOKE("plugin:notification|request_notification_permission");
},
async checkNotificationPermission() : Promise<NotificationPermission> {
    return await TAURI_INVOKE("plugin:notification|check_notification_permission");
},
async startDetectNotification() : Promise<null> {
    return await TAURI_INVOKE("plugin:notification|start_detect_notification");
},
async stopDetectNotification() : Promise<null> {
    return await TAURI_INVOKE("plugin:notification|stop_detect_notification");
}
}

/** user-defined events **/



/** user-defined constants **/



/** user-defined types **/

export type NotificationPermission = "Granted" | "NotGrantedAndShouldRequest" | "NotGrantedAndShouldAskManual"

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
