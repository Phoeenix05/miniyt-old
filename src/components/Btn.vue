<script setup>
// export default {
//     props: {
//         title: string,
//         path: string
//     }
// }

const props = defineProps([ "title", "path" ])
</script>

<template>
    <nuxt-link 
        class="flex items-center p-2 px-3 m-4 font-semibold tracking-wider text-center text-white duration-75 border-2 rounded-md w-fit bg-neutral-800 border-neutral-700 hover:bg-neutral-700" 
        :to="path"
    >
        {{ title }}
    </nuxt-link>
</template>
