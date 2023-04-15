// import { BaseDirectory, createDir, exists, readTextFile, writeTextFile } from "@tauri-apps/api/fs"

export interface IChannelList {
    profiles?: Profile[]
}

export interface Profile {
    name: string
    channels: Channel[]
}

export interface Channel {
    name: string
    url: string
}

// async function create_data_file(): Promise<Boolean> {
//     try {
//         await createDir("data", { dir: BaseDirectory.AppData, recursive: true })
//         await writeTextFile("data/channels.json", "{}", { dir: BaseDirectory.AppData })
//         return true
//     } catch (e) {
//         console.log(`Error occurred while creating data file:\n ${e}`)
//         return false
//     }
// }

// export async function file_exists(file: string): Promise<Boolean> {
//     if (!await exists(file, { dir: BaseDirectory.AppData })) {
//         return false
//     }
//     return true
// }

// export async function get_data(): Promise<IChannelList> {
//     if (!await file_exists("data/channels.json")) {
//         return { profiles: undefined }
//     }
//     if (!await create_data_file()) {
//         return { profiles: undefined }
//     }

//     const file_data = await readTextFile("data/channels.json", { dir: BaseDirectory.AppData })
//     const json: IChannelList = JSON.parse(file_data)
//     return json
// }

// export async function update_data(channel: Channel): Promise<Boolean> {
//     let data = await get_data()
//     if (!data || !data?.profiles) {
//         return false
//     }

//     data.profiles[0].channels.push(channel)
//     try {
//         await writeTextFile("data/channels.json", JSON.stringify(data), { dir: BaseDirectory.AppData })
//     } catch (e) {
//         console.log(`Error occurred while trying to update save data:\n ${e}`)
//         return false
//     }
//     return true
// }