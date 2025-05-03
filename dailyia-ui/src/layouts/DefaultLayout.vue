<template>
  <div class="layout">
    <header  class="card-block p-4 mb-6 flex justify-between items-center">
      <h1 class="text-lg underline">DAILYiA</h1>
      <nav class="p-4 space-x-4 text-sm">
        <router-link to="/">Home</router-link>
        <router-link to="/about">About</router-link>
        <router-link to="/contact">Contact</router-link>
      </nav>
    </header>
    <main>
      <slot />
    </main>
  </div>
</template>

<style lang="scss" scoped>

</style>
