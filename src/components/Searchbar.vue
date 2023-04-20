<script setup>
// let shown = false

// addEventListener("keypress", (event) => {
//     // console.log(event.key, event.metaKey)
    
//     if (event.metaKey ) {
//         shown = !shown
//     }
// })
let shown = false
document.onkeydown = (e) => {
    if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        console.log("a")

        shown = !shown
    }
}
</script>

<template>
    <!-- <div @click.meta="shown = !shown"></div> -->
    <div v-if="shown">
        <h1>Hallo</h1>
    </div>
</template>