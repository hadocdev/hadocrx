<script>
	import BanglaTextArea from "$lib/components/BanglaTextArea.svelte";
	import { Button } from "$lib/components/ui/button/index.js";
	import { Input } from "$lib/components/ui/input/index.js";
	import { Label } from "$lib/components/ui/label/index.js";
	import { Separator } from "$lib/components/ui/separator/index.js";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";
	import { CreateProfile } from "$lib/wailsjs/go/main/App.js";

	let name = $state("");
	let left = $state("");
	let right = $state("");
	let bottom = $state("");
	let disabled = $derived.by(() => {
		return (name === "") 
		|| (left === "")
		|| (right === "")
		|| (bottom === "");
	});

	function createProfile() {
		CreateProfile(name, left, right, bottom).then(
			(success) => {
				if (!success) alert("Error saving profile.");
		});
	}

</script>

<div class="px-2 pt-4 flex flex-col items-center gap-8 mx-4">
	<div class="w-full mb-[-1rem]">
		<p class="text-lg font-semibold">Create New Profile</p>
		<Separator class="mt-2 mb-4" />
	</div>
	<div class="w-full flex flex-col gap-3">
		<Label for="profilename">Name</Label>
		<Input id="profilename" type="text" class="max-w-md" placeholder="Name of the profile" bind:value={name} />
	</div>
	<div class="flex w-full gap-6">
		<BanglaTextArea id="doctorinfoleft" label="Left" placeholder="This will go on the top left." subtext="Ctrl+M to switch between Bangla and English" height="36" checked bind:value={left} />
		<BanglaTextArea id="doctorinforight" label="Right" placeholder="This will go on the top right." subtext="Ctrl+M to switch between Bangla and English" height="36" bind:value={right} />
	</div>
	<div class="w-full">
		<BanglaTextArea id="bottomtext" label="Bottom" placeholder="This will go on the bottom." subtext="Ctrl+M to switch between Bangla and English" height="18" checked bind:value={bottom} />
	</div>
	<Button class="uppercase w-48 mt-8" onclick={createProfile} disabled={disabled}>Create</Button>
</div>