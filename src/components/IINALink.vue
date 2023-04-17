<script>
export default {
    props: [
        "video_id"
    ]
}
</script>

<template>
    <div class=" w-fit p-3 m-2 bg-blue-500 rounded-lg font-semibold text-white">
        <nuxt-link :to="'iina://weblink?url=https://www.youtube.com/watch?v=' + video_id">Watch in IINA</nuxt-link>
    </div>
</template>
