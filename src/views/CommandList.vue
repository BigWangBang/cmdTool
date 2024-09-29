<script lang="ts" setup>
import { reactive, onBeforeMount, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessage } from 'element-plus'
import type { FormInstance } from 'element-plus'


interface CommandData {
    name: string,
    command?: string,
    subCommand?: DomainItem[],
}


interface DomainItem {
    value: string
}

const formData = ref({ name: "", command: "" })

const commandList = ref([]);

//默认首页的formData
const groupFormData = reactive<CommandData>({
    name: '',
    subCommand: [
        {
            value: '',
        },
    ],
})




const groupFormRef = ref<FormInstance>()

//是否显示groupDialog
const showGroupAdd = ref(false)


// command执行结果
const dialogContent = ref("")
const dialogVisible = ref(false)

const loading = ref(false)



onBeforeMount(() => {
    loadTodos();
})

const addCommand = () => {
    if (formData.value.name.trim() !== '') {
        commandList.value.push(<CommandData>{
            name: formData.value.name,
            command: formData.value.command,
        })
        formData.value = { name: "", command: "" }; // Clear the input box
        saveTodos();
    }
}

const submitGroup = () => {
    showGroupAdd.value = false;
    commandList.value.push({
        name: groupFormData.name,
        subCommand: groupFormData.subCommand
    })
    saveTodos();
    groupFormData = {
        name: '',
        subCommand: [
            {
                value: '',
            },
        ],
    }
}
const removeDomain = (item: DomainItem) => {
    const index = groupFormData.subCommand.indexOf(item)
    if (index !== -1) {
        groupFormData.subCommand.splice(index, 1)
    }
}
const addSubCommand = () => {
    groupFormData.subCommand.push({
        value: '',
    })
}
async function deleteCommand(index) {
    commandList.value.splice(index, 1);
    saveTodos();
}
async function saveTodos() {
    localStorage.setItem('command', JSON.stringify(commandList.value));
}
async function loadTodos() {
    const savedTodos = localStorage.getItem('command');
    if (savedTodos) {
        commandList.value = JSON.parse<CommandData>(savedTodos);
    }
}

async function runRow(action: number) {
    var command: CommandData = commandList.value[action];
    if (command.command?.length) {
        ElMessage("command" + command.command);
        loading.value = true;
        var resultMsg = await invoke("run", { name: command.command });
        loading.value = false;
        dialogContent.value = resultMsg;

        if (dialogContent.value) {
            dialogVisible.value = true;
        }
    } else {
        loading.value = true;
        let results = await Promise.all(command.subCommand?.map(async (item) => {
            var result = await invoke("run", { name: item.value });
            return result;
        }));
        loading.value = false;
        dialogContent.value = results.join('\n');
        if (dialogContent.value) {
            dialogVisible.value = true;
        }
    }
}


const resetGroupForm = (formEl: FormInstance | undefined) => {
    if (!formEl) return
    formEl.resetFields()
}

// 格式化 subCommand 数组为一个字符串
const formatSubCommands = (subCommands?: DomainItem[]): string => {
    if (!subCommands || subCommands.length === 0) {
        return '';
    }
    return subCommands.map(item => item.value).join('\n');
};


defineExpose({ formatSubCommands });
</script>


<template>
    <div class="todo-app">

        <el-form :inline="true" class="demo-form-inline">
            <el-form-item label="添加命令">
                <el-input id="greet-input" v-model="formData.command" placeholder="adb *" clearable />
            </el-form-item>
            <el-form-item label="别名" style="width: 180px;">
                <el-input id="nick-input" v-model="formData.name" placeholder="" clearable />
            </el-form-item>
            <el-form-item>
                <el-button type="primary" @click="addCommand">添加</el-button>
            </el-form-item>

            <el-form-item>
                <el-button type="primary" fixed="right" @click="showGroupAdd = true">添加Gruop</el-button>
            </el-form-item>
        </el-form>

        <el-table v-loading="loading" :data="commandList" :row-style="{ height: '50px' }">
            <el-table-column prop="name" label="别名" width="180" />
            <el-table-column label="命令" width="600" show-overflow-tooltip>
                <template #default="{ row }">
                    <span v-if="row.command">{{ row.command }}</span>
                    <span class="single-line-text" v-else>
                        {{ formatSubCommands(row.subCommand) }}
                    </span>
                </template>
            </el-table-column>
            <el-table-column fixed="right" label="操作" min-width="120">
                <template #default="scope">
                    <el-button link type="primary" size="small" @click.prevent="deleteCommand(scope.$index)">
                        Delete
                    </el-button>
                    <el-button link type="primary" size="small" @click="runRow(scope.$index)">Run</el-button>
                </template>
            </el-table-column>
        </el-table>
    </div>

    <el-dialog v-model="dialogVisible" title="执行结果" width="600" :before-close="handleClose">
        <span style="white-space: pre-wrap; margin-top: auto;">{{ dialogContent }}</span>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="dialogVisible = false">
                    OK
                </el-button>
            </div>
        </template>
    </el-dialog>



    <el-dialog v-model="showGroupAdd" title="添加组" width="500" :before-close="handleClose">
        <el-form ref="groupFormRef" class="demo-form-inline" :model="groupFormData">

            <el-form-item prop="name" label="分组名" :rules="[
                {
                    required: true,
                    message: '请输入分组名',
                    trigger: 'blur',
                },
            ]">
                <el-input v-model="groupFormData.name" />
            </el-form-item>

            <div>命令</div>

            <el-form-item v-for="(domain, index) in groupFormData.subCommand" :prop="'subCommand.' + index + '.value'"
                :rules="{
                    required: true,
                    message: 'domain can not be null',
                    trigger: 'blur',
                }">
                <el-input v-model="domain.value" />
                <el-button class="mt-2" @click.prevent="removeDomain(domain)" style="margin-left: 10px;">
                    Delete
                </el-button>
            </el-form-item>

            <el-form-item>
                <el-button type="primary" @click="submitGroup">确定</el-button>
                <el-button @click="addSubCommand">添加Gruop命令</el-button>
                <el-button @click="resetGroupForm(groupFormRef)">Reset</el-button>
            </el-form-item>

        </el-form>
    </el-dialog>
</template>

<style scoped>
/* Add some basic styling */
.todo-app {
    max-width: 2200px;
    margin: 20px auto;
    padding: 20px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.el-table .el-table__row {
    height: 50px;
    /* 你想要设置的行高 */
}

input[type="checkbox"] {
    margin-right: 10px;
}

.demo-form-inline .el-input {
    --el-input-width: 220px;
}

.demo-form-inline .el-select {
    --el-select-width: 220px;
}

.single-line-text {
    white-space: pre-wrap;
    word-break: break-all;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
}
</style>