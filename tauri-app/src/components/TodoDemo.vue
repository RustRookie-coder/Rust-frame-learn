<script lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {defineComponent, onMounted, ref} from "vue";

export default defineComponent({
  setup() {
    const todos = ref<String[]>([]);
    const newTodo = ref<string>('');
    // 从 rust 后端添加新的待办事项
    const addTodo = async () => {
      if(newTodo.value) {
        const todo = await invoke<string>('add_todo', { todo: newTodo.value })
        todos.value.push(todo);
        newTodo.value = '';
      }
    };

    //页面加载时获取所有的待办事项
    onMounted(async () => {
      todos.value = await invoke<string[]>('get_todos');
    });

    return {
      todos,
      newTodo,
      addTodo,
    };
  },
});
</script>

<template>
  <div id="app">
    <h1>Todo List</h1>
    <form @submit.prevent="addTodo">
      <input v-model="newTodo" placeholder="Enter a todo ...."/>
      <button type="submit">Add Todo</button>
    </form>
    <ul>
      <li v-for="(todo, index) in todos" :key="index"> {{ todo }}</li>
    </ul>
  </div>
</template>

<style scoped>
#app {
  font-family: Arial, sans-serif;
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