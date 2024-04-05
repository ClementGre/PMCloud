<script setup lang="ts">
definePageMeta({
  layout: 'default',
  middleware: ['root-server-only']
})

const switchUserId = () => {
  let userId = useCookie('pm_user_id')
  if(userId.value == '9'){
    userId.value = '10'
  }else{
    userId.value = '9'
  }
}


</script>

<template>
  <main>
    <h1>about</h1>
    <Button @click="switchUserId">Switch userId</Button>
    <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore
      magna aliqua. Morbi non arcu risus quis. Morbi tristique senectus et netus et. Odio tempor orci dapibus ultrices
      in iaculis nunc sed augue. Diam sollicitudin tempor id eu nisl nunc mi ipsum. Pharetra diam sit amet nisl. Non
      tellus orci ac auctor augue. Arcu non odio euismod lacinia at quis risus. Faucibus vitae aliquet nec ullamcorper
      sit amet risus nullam. Iaculis nunc sed augue lacus. Aliquam purus sit amet luctus venenatis lectus magna
      fringilla. Enim nulla aliquet porttitor lacus. Dolor morbi non arcu risus quis varius. Venenatis urna cursus eget
      nunc scelerisque viverra mauris in. Quis eleifend quam adipiscing vitae proin sagittis nisl rhoncus. Mattis
      molestie a iaculis at erat pellentesque adipiscing commodo elit. Amet mauris commodo quis imperdiet massa
      tincidunt nunc. Leo integer malesuada nunc vel risus commodo viverra maecenas accumsan. Amet aliquam id diam
      maecenas ultricies mi eget mauris pharetra.

      Sollicitudin tempor id eu nisl nunc mi ipsum faucibus. Tincidunt eget nullam non nisi est. Pellentesque habitant
      morbi tristique senectus et netus et malesuada fames. Eu volutpat odio facilisis mauris sit amet. Nulla facilisi
      nullam vehicula ipsum a arcu cursus. Volutpat lacus laoreet non curabitur gravida arcu ac tortor dignissim. Nisi
      scelerisque eu ultrices vitae auctor eu. Auctor augue mauris augue neque. Magnis dis parturient montes nascetur
      ridiculus. Phasellus egestas tellus rutrum tellus pellentesque. Id aliquet risus feugiat in. Sit amet nulla
      facilisi morbi tempus iaculis urna. Donec ac odio tempor orci dapibus ultrices in iaculis. Sem et tortor consequat
      id porta nibh venenatis. Ut diam quam nulla porttitor massa id. Felis eget nunc lobortis mattis. Praesent
      elementum facilisis leo vel fringilla est ullamcorper eget. Tristique risus nec feugiat in fermentum. Fames ac
      turpis egestas sed tempus urna.

      Non odio euismod lacinia at quis. Odio tempor orci dapibus ultrices in iaculis nunc sed. Elementum sagittis vitae
      et leo. Gravida arcu ac tortor dignissim convallis aenean et tortor at. Lectus mauris ultrices eros in. Non enim
      praesent elementum facilisis leo vel fringilla. Enim nunc faucibus a pellentesque sit amet porttitor eget. Libero
      enim sed faucibus turpis in eu mi bibendum. Scelerisque eleifend donec pretium vulputate sapien nec sagittis. Enim
      diam vulputate ut pharetra sit amet aliquam id. Aliquam purus sit amet luctus venenatis lectus magna fringilla
      urna. Facilisi etiam dignissim diam quis enim. Leo vel orci porta non pulvinar neque laoreet suspendisse interdum.
      Natoque penatibus et magnis dis parturient. Nibh tortor id aliquet lectus proin nibh nisl.
    </p>
  </main>
</template>

<style scoped lang="stylus">

</style>
