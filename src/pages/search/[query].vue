<script setup>
definePageMeta({
    layout: "list"
})

const route = useRoute()

const { id } = route.params
</script>

<template>
    <NuxtLayout>
        <p>{{ id }}</p>
    </NuxtLayout>
</template>