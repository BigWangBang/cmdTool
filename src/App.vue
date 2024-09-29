<template>
  <!-- <el-hader>
    <el-menu :default-active="activeIndex" class="el-menu-demo" mode="horizontal" @select="handleSelect">
      <el-menu-item index="1">
        <RouterLink to="/">日志快查</RouterLink>
      </el-menu-item>
      <el-menu-item index="2">
        <RouterLink to="/command">命令行工具</RouterLink>
      </el-menu-item>
    </el-menu>
  </el-hader> -->

  <main>
    <RouterView />
  </main>
</template>

<script setup>

</script>
<style scoped>
.router-link-active,
a {
  text-decoration: none;
}
</style>
