<script setup lang="ts">
import { BaseDirectory, createDir, exists, readTextFile, writeTextFile } from "@tauri-apps/api/fs";

async function refresh_videos() {
    if (!await exists("data/channels.json", { dir: BaseDirectory.AppData })) {
        await createDir("data", { dir: BaseDirectory.AppData, recursive: true })
        await writeTextFile("data/channels.json", "{}", { dir: BaseDirectory.AppData })
    }
    const file_data = await readTextFile("data/channels.json", { dir: BaseDirectory.AppData })
    const json: IJsonData = JSON.parse(file_data)
    return json
}

interface IJsonData {
    profiles: [
        {
            name: string,
            channels: string[]
        }
    ]
}

let { data, pending } = useAsyncData("channels", () => refresh_videos().then(data => data))
let refresh = () => refreshNuxtData("channels")
</script>

<template>
    <div class="absolute top-0 right-0">
        <button @click="refresh">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
            </svg>
        </button>
    </div>
    <div>
        <p v-for="channel in data?.profiles[0].channels" v-bind:key="channel">
            {{ channel }}
        </p>
    </div>
</template>