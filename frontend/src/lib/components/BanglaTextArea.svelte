<script>
	import { Label } from "$lib/components/ui/label/index.js";
	import { Textarea } from "$lib/components/ui/textarea/index.js";
	import { Switch } from "$lib/components/ui/switch/index.js";
	import { myAvro } from "$lib/avro.js";
	import { onMount } from "svelte";
	
	let {
		id, label, placeholder, subtext, 
		checked, height, value = $bindable(), ...restProps
	} = $props();
	let bangla = $state(checked ?? false);
	let textAreaElement;

	onMount(() => {
		textAreaElement = document.getElementById(id);
		myAvro(textAreaElement);
		textAreaElement.addEventListener('keydown', (e) => {
			const keycode = e.which || e.keyCode;
			if (keycode === 77 && e.ctrlKey && !e.altKey && !e.shiftKey) {
		        e.preventDefault();
		        bangla = textAreaElement.bangla;
		        return false;
		    }
		});
	});

	$effect(() => {
		textAreaElement.bangla = bangla;
	});
</script>

<div class="grid w-full gap-3">
  <div class="flex items-center justify-between">
  	<Label>{label}</Label>
  	<div class="flex items-center space-x-2 ">
  		<Switch bind:checked={bangla}/>
  		<Label>Bangla</Label>
  	</div>
  </div>
  <Textarea placeholder="{placeholder}" class="resize-none font-['SolaimanLipi'] h-{height}" id="{id}" bind:value {...restProps} />
  <p class="text-muted-foreground text-sm mt-[-0.5rem] mx-2">
    {subtext}
  </p>
</div>
