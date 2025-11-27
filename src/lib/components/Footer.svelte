<script lang="ts">
  /**
   * The text to display inside the button.
   * @type {string}
   * @default 'Submit'
   */
  export let buttonText: string = "Submit";

  /**
   * If true, disables the button.
   * @type {boolean}
   * @default false
   */
  export let disabled: boolean = false;
</script>

<footer
  class="flex-none left-0 right-0 p-4 bg-base-200/80 backdrop-blur-sm border-t border-base-300"
>
  <div class="container mx-auto">
    <button class="btn btn-primary btn-block" on:click {disabled}>
      {buttonText}
    </button>
  </div>
</footer>
