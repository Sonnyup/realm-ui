<template>
    <n-config-provider style="height: 100%;background-color: #f1f1f1;">
        <n-message-provider>
            <content></content>
        </n-message-provider>
    </n-config-provider>
</template>

<script setup lang="ts">
import Content from '@/components/Content.vue';
import {
    NMessageProvider,
    NConfigProvider
} from 'naive-ui';

</script>

<style lang="scss">
#app {
    background-color: #f1f1f1;
}
</style>