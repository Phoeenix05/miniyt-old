<script setup>
const id = ref("QRDmDPFySlU")
</script>

<template>
    <NuxtLayout>
        <!-- <Btn title="Hallo" path="/search/Sword Art Online" /> -->
        <input :value="id" type="text">
        <nuxt-link :to="'/video/' + id">Search</nuxt-link>
        <br>
        <nuxt-link to="/video/QRDmDPFySlU">Watch</nuxt-link>
    </NuxtLayout>
    <!-- <div>
        <br>
        <nuxt-link to="/videos">Videos</nuxt-link>
        <br>
        <nuxt-link to="/search/Sword Art Online">Search</nuxt-link>
    </div> -->
</template>

<style>
html {
    background-color: theme("colors.dark.900");
}
</style>