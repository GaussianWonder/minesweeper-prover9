<script setup lang="ts">
const keep = ref('')
</script>

<template>
  <div class="px-5 py-2 rounded-lg shadow-emerald-200 shadow-md flex flex-row gap-2">
    <AltButton>
      AltButton
    </AltButton>
    <DangerButton>
      DangerButton
    </DangerButton>
    <PrimaryButton>
      PrimaryButton
    </PrimaryButton>
    <SuccessButton>
      SuccessButton
    </SuccessButton>
    <WarnButton>
      WarnButton
    </WarnButton>
  </div>
  <div class="px-5 py-2 rounded-lg shadow-emerald-200 shadow-md flex flex-row gap-2">
    <AltButton
      :is-tiny="true"
    >
      Alt
    </AltButton>
    <DangerButton
      :is-tiny="true"
    >
      Danger
    </DangerButton>
    <PrimaryButton
      :is-tiny="true"
    >
      Primary
    </PrimaryButton>
    <SuccessButton
      :is-tiny="true"
    >
      Success
    </SuccessButton>
    <WarnButton
      :is-tiny="true"
    >
      Warn
    </WarnButton>
  </div>
  <div class="px-5 py-2 rounded-lg shadow-emerald-200 shadow-md flex flex-col gap-2">
    <AltInput v-model="keep">
      <template #label>
        AltInput
      </template>
    </AltInput>
    <DangerInput v-model="keep">
      <template #label>
        DangerInput
      </template>
    </DangerInput>
    <PrimaryInput v-model="keep">
      <template #label>
        PrimaryInput
      </template>
    </PrimaryInput>
    <SuccessInput v-model="keep">
      <template #label>
        SuccessInput
      </template>
    </SuccessInput>
    <WarnInput v-model="keep">
      <template #label>
        WarnInput
      </template>
    </WarnInput>
  </div>
  <PrimaryInput type="file">
    <template #label>
      File Input
    </template>
  </PrimaryInput>
</template>
