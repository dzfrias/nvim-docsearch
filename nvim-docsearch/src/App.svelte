<script lang="ts">
  import Input from "./lib/Input.svelte";
  import sites from "./assets/sites.txt?raw";

  const data = sites.split("\n").map((site) => ({
    title: decodeURIComponent(site).split("#")[1],
    link: site,
  }));
</script>

<main>
  <div class="center">
    <h1>
      <a
        href="https://neovim.io"
        target="_blank"
        rel="noreferrer"
        class="first-letter">Neovim</a
      >
      Docs
    </h1>
    <div>
      <Input {data} />
    </div>
  </div>
</main>

<style>
  a {
    color: var(--clr-primary);
    text-decoration: none;
    position: relative;
    top: 0;
    transition: top ease 300ms;
  }
  a:hover {
    top: -5px;
    text-decoration: underline;
    text-decoration-color: var(--clr-secondary);
  }
  a:visited {
    color: var(--clr-primary);
  }

  .first-letter {
    display: inline-block;
  }
  .first-letter::first-letter {
    color: var(--clr-secondary) !important;
  }
</style>
