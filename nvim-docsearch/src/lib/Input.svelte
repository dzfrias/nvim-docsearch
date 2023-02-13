<script lang="ts">
  import Fuzzy from "svelte-fuzzy";

  type DataEntry = {
    title: string;
    link: string;
  };

  export let data: DataEntry[];

  // svelte-fuzzy variables
  let query = "";
  let options = { keys: ["title"] };
  let formatted = [];
  let result = [];

  let selected = 0;
  // Reset selected every time result box changes
  $: result, (selected = 0);
  // Selected can't be greater than result length or less than 0
  $: if (selected > result.length - 1 || selected < 0) {
    selected = 0;
  }
  const CUTOFF = 30;
  $: formatted = formatted.slice(0, CUTOFF);

  function open(idx: number = null) {
    let current_link = (result[idx ?? selected] ?? { link: "" }).link;
    if (current_link == "") {
      return;
    }
    window.open(current_link, "_blank");
  }
</script>

<svelte:window
  on:keydown={(event) => {
    if (event.key === "Enter") {
      open();
    }

    if (
      (!event.ctrlKey && event.key != "ArrowDown" && event.key != "ArrowUp") ||
      result.length == 0
    ) {
      return;
    }
    switch (event.key) {
      case "ArrowDown":
      case "n":
        selected++;
        event.preventDefault();
        let nextSelected = document.querySelector(
          'li[aria-selected="true"] + li'
        );
        if (nextSelected !== null) {
          nextSelected.scrollIntoView(false);
        }
        break;

      case "ArrowUp":
      case "p":
        selected--;
        event.preventDefault();
        let resultBox = document.querySelector(".result-box");
        let selectedElem = document.querySelector('li[aria-selected="true"]');
        if (
          selectedElem.getBoundingClientRect().top - 5 <
            resultBox.getBoundingClientRect().top &&
          selectedElem.previousElementSibling != null
        ) {
          selectedElem.previousElementSibling.scrollIntoView();
        }
        break;

      default:
        return;
    }
  }}
/>

<input type="text" bind:value={query} />
<Fuzzy {query} {data} {options} bind:formatted bind:result />

<div class="result-box">
  {#each formatted as item, idx}
    {#each item as line}
      {#if idx == selected}
        <li
          aria-selected="true"
          on:click={() => open()}
          on:keydown={() => {}}
          id={idx.toString()}
        >
          {#each line as { matches, text }}
            {#if matches}
              <mark>{text}</mark>
            {:else}
              {text}
            {/if}
          {/each}
        </li>
      {:else}
        <li
          on:click={(event) => open(parseInt(event.currentTarget.id))}
          on:keydown={() => {}}
          id={idx.toString()}
        >
          {#each line as { matches, text }}
            {#if matches}
              <mark>{text}</mark>
            {:else}
              {text}
            {/if}
          {/each}
        </li>
      {/if}
    {/each}
  {/each}
</div>

<style>
  mark {
    background-color: transparent;
    color: var(--clr-secondary);
  }

  .result-box {
    text-align: start;
    max-height: 10rem;
    min-height: 10rem;
    overflow-y: scroll;
    -ms-overflow-style: none;
    scrollbar-width: none;
    border: 1px solid var(--clr-secondary);
    border-radius: 5px;
    max-width: 20rem;
    margin: 0.3rem auto;
    color: var(--clr-fg);
  }

  .result-box::-webkit-scrollbar {
    display: none;
  }

  .result-box li {
    overflow-wrap: anywhere;
    list-style: none;
    padding-left: 0.2rem;
  }

  .result-box li:hover {
    cursor: pointer;
    border: 1px solid lightblue;
  }

  .result-box li[aria-selected="true"] {
    background-color: var(--clr-primary);
    color: black;
  }

  .result-box li[aria-selected="true"] mark {
    color: black;
  }

  input[type="text"] {
    -webkit-border-radius: 20px;
    -moz-border-radius: 20px;
    border-radius: 20px;
    border: 1px solid grey;
    color: grey;
    width: min(90vw, 20rem);
    font-size: 1.2em;
    height: 2rem;
    padding-left: 0.7rem;
  }

  input[type="text"]:focus {
    outline: none;
    border: 1px solid var(--clr-secondary);
    color: var(--clr-fg);
  }
</style>
