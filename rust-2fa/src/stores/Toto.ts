import { defineStore } from 'pinia';
import {ref} from "vue";

export const useTotoStore = defineStore('toto', ()=>{
    const company = ref("");
    const accountName = ref("");
    const otpauth = ref("");
    const previewUrl = ref<string>('');
    const remainingTime = ref(0);
    const dataList = ref();
    return {company,accountName,otpauth,previewUrl,remainingTime,dataList};
});