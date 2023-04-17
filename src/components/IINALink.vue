<script>
export default {
    props: [
        "video_id"
    ]
}
</script>

<template>
    <div>
        <nuxt-link :to="'iina://weblink?url=https://www.youtube.com/watch?v=' + video_id">Watch in IINA</nuxt-link>
    </div>
</template>
