<script setup>
const route = useRoute()

const { id } = route.params
const url = "https://www.youtube.com/embed/" + id
</script>

<template>
    <iframe 
        width="560" 
        height="315" 
        :src="url" 
        title="YouTube video player" 
        frameborder="0" 
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
        allowfullscreen></iframe>
    <IINALink :video_id="id"/>
</template>