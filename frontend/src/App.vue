<script setup lang="ts">
import AppNavbar from './components/layout/AppNavbar.vue'
import AppSidebar from './components/layout/AppSidebar.vue'
</script>

<template>
  <div class="min-h-screen flex flex-col bg-slate-950 text-slate-100">
    <AppNavbar />
    <div class="flex-1 flex">
      <AppSidebar />
      <main class="flex-1 p-6 md:p-8 overflow-y-auto max-w-7xl mx-auto w-full">
        <router-view />
      </main>
    </div>
  </div>
</template>
