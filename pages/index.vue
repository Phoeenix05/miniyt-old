<!-- <script src="ts">
<script setup lang="ts">
// const playlists = $fetch("/api/playlists").then(res => res)

// const props = {
//     playlists: playlists
// }

// const { data, refresh, pending } = useAsyncData("/api/playlists", () => $fetch("/api/playlists"))

// const props = {
//     playlists: data
// }
// export {}

// const { data: playlists } = await useFetch(
//     "https://localhost/api/playlists"
// )

// export {}
</script> -->

<!-- <script lang="js">
// export default ({
//     data() {
//         return {
//             playlists: ""
//         }
//     },
//     // data: () => ({
//     //     playlists: Object
//     // }),
//     async fetch() {
//         // this.playlists = await fetch("https://localhost:8000/api/playlists")
//         //     .then(res => res.json())
//         // console.log(playlists)
        
//         // if (playlists) {
//         //     return {
//         //         playlists: playlists
//         //     } 
//         // } else {
//         //     return redirect("/")
//         // }
//         const playlists = await fetch("https://localhost:8000/api/playlists").then(res => res.json())
//         console.log(playlists)
//         this.playlists = playlists
//     },
//     fetchOnServer: false
// })
</script> -->

<script setup>
import { invoke } from "@tauri-apps/api/tauri"
// const { data, pending, refresh } = await useFetch("https://localhost:8000/api/playlists")
const data = await invoke("hallo")
</script>

<template>
    <div>
        <!-- <NuxtWelcome /> -->
        <!-- <p v-if="$fetchState.pending">Fetching</p> -->
        <!-- <p>{{ playlists }}</p> -->
        {{ data }}
        <!-- <div>
            <button :disabled="pending" @click="refresh">
                get latest data
            </button>
        </div> -->
    </div>
</template>
