<template>
  <div class="card-block p-4 col-span-2">
    <h2 class="text-sm mb-4">TODAY'S QUESTS</h2>
    <div class="flex mb-4">
      <DTextField v-model="newQuest" />
      <DBtn class="ml-2" @click="addQuest" :disabled="newQuest.length === 0">Add</DBtn>
    </div>
    <ul class="space-y-3 text-xs">
        <template v-for="(item, index) in quests" :key="index">
            <li><input type="checkbox" class="mr-2" />{{ item.text }}</li>            
        </template>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface QuestItem {
    text: string;
}

const quests = ref<QuestItem[]>([
    { text: "Brew health potion (morning tea)"},
    { text: "Study spellbook (read 20 pages)"},
    { text: "Write magic scroll (code for 2h)"},
]);

const addQuest = () => {
    quests.value.push({ text: newQuest.value });
    newQuest.value = "";
};

const newQuest = ref("");
</script>
