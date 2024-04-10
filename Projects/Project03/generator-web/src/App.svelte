<script lang="ts">
  import init, { Generator } from "generator";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Separator } from "$lib/components/ui/separator";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { ModeWatcher } from "mode-watcher";
  import { CircleHelp, Github } from "lucide-svelte";
  import { Confetti } from "svelte-confetti";
  import ToggleConfetti from "$lib/ToggleConfetti.svelte";
  import documentation from "/Project03.pdf?url";

  const version = __APP_VERSION__;

  let ntustId = "B12345678";
  let a = "";
  let b = "";
  let c = "";
  let d = "";
  let plainText = "";

  let copyButtonText = "Copy";

  onMount(async () => {
    await init();
  });

  function handleSubmit() {
    let generator = new Generator(ntustId);
    let result = generator.generate();

    a = result.a().toString();
    b = result.b().toString(2);
    c = result.c().toString(2).padStart(16, "0");
    d = result.d().toString(2).padStart(16, "0");

    plainText = result.to_plain_text();
  }

  function handleClear() {
    a = "";
    b = "";
    c = "";
    d = "";
    plainText = "";
  }

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(plainText);
      copyButtonText = "Copied!";
      setTimeout(() => (copyButtonText = "Copy"), 1000);
    } catch (err) {
      console.error("Failed to copy: ", err);
      copyButtonText = "Failed to copy!";
    }
  }
</script>

<ModeWatcher />

<main class="container mx-auto py-10">
  <div class="text-center">
    <h1><span class="text-gradient">NTUST</span> ID Booth's Multiplication</h1>
  </div>

  <div class="h-10" />

  <div class="flex flex-col items-center">
    <div class="w-full max-w-3xl">
      <form
        class="flex flex-col items-center w-full gap-1.5"
        on:submit|preventDefault={handleSubmit}
      >
        <div class="flex w-full flex-col gap-1.5">
          <Label for="ntust-id">NTUST ID</Label>
          <Input
            type="text"
            id="ntust-id"
            placeholder="B12345678"
            pattern={"[A-Za-z][0-9]{8}"}
            title="Please enter the correct format (for example: B12345678)"
            required
            bind:value={ntustId}
          />
        </div>
        <div class="flex flex-row gap-1.5">
          <ToggleConfetti>
            <Button slot="label" type="submit">Generate</Button>

            <Confetti cone x={[-0.5, 0.5]} />
            <Confetti cone amount={10} x={[-1, -0.4]} y={[0.25, 0.75]} />
            <Confetti cone amount={10} x={[0.4, 1]} y={[0.25, 0.75]} />
          </ToggleConfetti>
          <Button variant="secondary" on:click={handleClear}>Clear</Button>
          <Button
            variant="ghost"
            size="icon"
            href={documentation}
            target="_blank"
          >
            <CircleHelp class="h-4 w-4" />
          </Button>
        </div>
      </form>

      <Separator class="my-4" />

      <div class="flex flex-col items-center w-full gap-1.5">
        <div class="grid grid-cols-4 w-full gap-4">
          <div class="flex w-full flex-col col-span-4 md:col-span-1 gap-1.5">
            <Label for="a">A</Label>
            <Input type="a" id="a" class="font-mono" bind:value={a} readonly />
          </div>
          <div class="flex w-full flex-col col-span-4 md:col-span-3 gap-1.5">
            <Label for="b">B</Label>
            <Input type="b" id="b" class="font-mono" bind:value={b} readonly />
          </div>
          <div class="flex w-full flex-col col-span-4 md:col-span-2 gap-1.5">
            <Label for="c">C</Label>
            <Input type="c" id="c" class="font-mono" bind:value={c} readonly />
          </div>
          <div class="flex w-full flex-col col-span-4 md:col-span-2 gap-1.5">
            <Label for="d">D</Label>
            <Input type="d" id="d" class="font-mono" bind:value={d} readonly />
          </div>
          <div class="grid w-full col-span-4 gap-1.5">
            <Label for="result">Result</Label>
            <Textarea
              id="result"
              class="font-mono min-h-96"
              value={plainText}
            />
          </div>
          <div class="grid w-full col-span-4 gap-1.5">
            <Button variant="ghost" on:click={handleCopy}
              >{copyButtonText}</Button
            >
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="h-10" />

  <footer class="flex flex-col items-center text-center muted font-mono">
    <div>
      Powered by <a href="https://webassembly.org/" target="_blank" class="link"
        >WebAssembly</a
      > technology
    </div>
    <div>
      <a href="https://github.com/CRT-HAO" target="_blank" class="link"
        >Hayden</a
      >
      ❤️
      <a href="https://www.rust-lang.org/" target="_blank" class="link">Rust</a>
      &
      <a href="https://svelte.dev/" target="_blank" class="link">Svelte</a>
    </div>
    <div class="flex flex-col items-center mt-4">
      <a
        href="https://github.com/CRT-HAO/CS2006301-Computer-Organization/tree/master/Projects/Project03"
        target="_blank"
        class="muted link"
      >
        <Github />
      </a>
      <div>Version {version}</div>
    </div>
  </footer>
</main>

<style lang="postcss">
</style>
