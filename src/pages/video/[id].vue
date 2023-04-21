<script setup>
const route = useRoute()

const { id } = route.params
const url = "https://www.youtube.com/embed/" + id
</script>

<template>
    <NuxtLayout>
        <div class="w-screen h-[90vh] flex justify-center items-center">
            <iframe 
                width="100%"
                height="100%"
                :src="url" 
                title="YouTube video player" 
                frameborder="0" 
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
                allowfullscreen></iframe>
        </div>
        <div>
            <!-- <IINALink :video_id="id"/> -->
            <Btn title="Open in IINA" :path="'iina://weblink?url=' + url" />
        </div>
    </NuxtLayout>
</template>