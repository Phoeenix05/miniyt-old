<script setup>
// import { invoke } from "@tauri-apps/api/tauri"

// const data = await invoke("search_query", { q: "Sword Art Online" })
// console.log(data)
</script>

<template>
    <searchbar />
    <div>
        <nuxt-link to="/video/QRDmDPFySlU">Watch</nuxt-link>
        <br>
        <nuxt-link to="/videos">Videos</nuxt-link>
        <br>
        <nuxt-link to="/search/Sword Art Online">Search</nuxt-link>
    </div>
</template>
