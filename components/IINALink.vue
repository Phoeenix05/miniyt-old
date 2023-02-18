<script>
export default {
    props: [
        "video_id"
    ]
}
</script>

<template>
    <div>
        <!-- <a :href="'iina://https://www.youtube.com/watch?v=' + video_id">Watch in IINA</a> -->
        <nuxt-link :to="'iina://weblink?url=https://www.youtube.com/watch?v=' + video_id">Watch in IINA</nuxt-link>
        <!-- <nuxt-link to="iina://weblink?url=https://youtu.be/KXRoWk-Ic5E">Watch in IINA</nuxt-link> -->
    </div>
</template>