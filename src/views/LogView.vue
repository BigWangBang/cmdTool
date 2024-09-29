<script lang="ts" setup>
import { reactive, onBeforeMount, ref } from "vue";
const liveLogEnable = ref(false)

const importLog = () => {


}


</script>

<template>
    <div class="logView">
        <div class="left-item">
            
        </div>
        <div class="right-item">
            <el-button type="primary" v-if="!liveLogEnable" @click="importLog">选择Log地址</el-button>
            <el-text style="margin-right: 10px;"> 实时Logcat</el-text>
            <el-switch v-model="liveLogEnable" size="large" />
        </div>
    </div>

</template>



<style scoped>
.logView {
    max-width: 2200px;
    margin: 20px auto;
    padding: 20px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    /* 垂直居中 */
}

.left-item {
    margin-right: auto;
}

.right-item {
    margin-left: auto;
}
</style>