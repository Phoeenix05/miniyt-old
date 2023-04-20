<script setup>
import { invoke } from "@tauri-apps/api/tauri";

const route = useRoute()

const { query } = route.params

const { pending, data } = await useLazyAsyncData('data', async () => await invoke("search_query", { q: query }))
// console.log(data)
</script>

<template>
    <div>
        {{ pending ? 'loading' : "loaded" }}
    </div>
</template>