<script lang="ts">
  import { blackSwanError } from "../../stores/stores";

  // listen for err store change
</script>

<svelte:head>
  <title>Error..</title>
</svelte:head>

<header class="p-5">
  <h1 class="text-center">
    Oh no! Looks like an unknown error has occurred...
  </h1>
</header>
<!--Description of the Error-->
<main class="h-[80vh] flex flex-col">
  <div class="flex-1"></div>
  <div>
    <div class="flex items-center justify-center h-16">
      {#if $blackSwanError}
        <div class="p-4">Status Code: {$blackSwanError.status}</div>
        <div class="py-8 overflow-auto">
          <p>Reason: {$blackSwanError.message}</p>
        </div>
      {/if}
    </div>
  </div>
</main>
