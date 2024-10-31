<script setup lang="ts">
import {ref, onMounted, warn} from 'vue';
import {event} from "@tauri-apps/api";
import {invoke} from "@tauri-apps/api/core";

// 待办事项列表和输入的待办事项
const todos = ref<string[]>([]);
const newTodo = ref<string>('');

// 添加新的待办事项
const addTodo = async () => {
  if (newTodo.value) {
    const todo = await invoke<string>('add_todo', {todo: newTodo.value});
    todos.value.push(todo);
    newTodo.value = '';
  }
};
// 页面加载时获取待办事项
onMounted(async () => {
  todos.value = await invoke<string[]>('get_todos');
});

const warn = async (message, event) => {
  // 这里可以访问原生事件
  if (event) {
    event.preventDefault()
  }
  alert(message)
}
</script>

<template>
  <div id="app">
    <el-header>Todo List</el-header>
    <el-form :inline="true" @submit.prevent="addTodo">
      <el-form-item>
        <el-input v-model="newTodo" placeholder="Enter a new todo..."/>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="addTodo">Add Todo</el-button>
      </el-form-item>
      <el-form-item>
        <el-button @click="(event) => warn('Form cannot be submitted yet.', event)">Submit</el-button>
      </el-form-item>
    </el-form>
    <el-col>
      <el-collapse v-for="(todo, index) in todos" :key="index">{{ todo }}</el-collapse>
    </el-col>
  </div>
</template>

<style scoped>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  margin-top: 30px;
}

input {
  padding: 8px;
  width: 300px;
}

button {
  padding: 8px 16px;
  margin-left: 10px;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  padding: 8px;
  background: #062aa3;
  margin: 8px 0;
}

</style>
