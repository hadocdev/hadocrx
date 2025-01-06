<script>
	import BanglaTextArea from "$lib/components/BanglaTextArea.svelte";
	import { Button } from "$lib/components/ui/button/index.js";
	import { Input } from "$lib/components/ui/input/index.js";
	import { Label } from "$lib/components/ui/label/index.js";
	import { Switch } from "$lib/components/ui/switch/index.js";
	import * as Select from "$lib/components/ui/select/index.js";
	import { Separator } from "$lib/components/ui/separator/index.js";
	import * as Dialog from "$lib/components/ui/dialog/index.js";
	import { goto } from "$app/navigation";
	import { LoadProfile } from "$lib/wailsjs/go/main/App.js";

	LoadProfile().then((profiles) => {
		console.log(profiles);
		if (profiles === null) {
			goto("/create-profile");
		}
	});

	let patientsex = $state("");
	let patientSexTriggerContent = $derived.by(() => {
		if (patientsex === "") return "Select a sex";
		return patientsex;
	});

	let isDialogOpen = $state(false);
	function openDialog() {
		isDialogOpen = true;
	}
</script>

<div class="px-2 pt-4 flex flex-col items-center gap-8 mx-4">
	<div class="w-full mb-[-1rem]">
		<Label>Doctor's Info</Label>
		<Separator class="mt-2" />
	</div>
	<div class="flex w-full gap-6">
		<BanglaTextArea label="Left" placeholder="This will go on the top left." subtext="Ctrl+M to switch between Bangla and English" id="doctorinfoleft" checked />
		<BanglaTextArea label="Right" placeholder="This will go on the top right." subtext="Ctrl+M to switch between Bangla and English" id="doctorinforight" />
	</div>
	<div class="w-full mb-[-1rem]">
		<Label>Patient's Info</Label>
		<Separator class="mt-2" />
	</div>
	<div class="flex w-full">
		<div class="flex w-full max-w-xs flex-col gap-3">
		  <Label for="patientname">Name</Label>
		  <Input type="text" id="patientname" placeholder="Patient's Name" />
		</div>
	</div>
	<div class="flex w-full gap-6 items-center">
		<div class="flex max-w-xs flex-col gap-3">
		  <Label>Date of birth</Label>
		  <div class="flex gap-1">
		  	<Input type="number" class="remove-arrow" id="date" placeholder="Date" />
		  	<Input type="number" class="remove-arrow" id="month" placeholder="Month" />
		  	<Input type="number" class="remove-arrow" id="year" placeholder="Year" />
		  </div>
		</div>
		<div class="flex max-w-xs flex-col gap-3">
		  <Label for="patientweight">Weight</Label>
		  <Input type="number" class="remove-arrow" id="patientweight" placeholder="in kg" />
		</div>
		<div class="flex flex-col gap-3 w-full">
			<Label>Sex</Label>
			<Select.Root type="single" bind:value={patientsex}>
			  <Select.Trigger class="w-[180px]">
			  	{patientSexTriggerContent}
			  </Select.Trigger>
			  <Select.Content>
			    <Select.Item value="Male">Male</Select.Item>
			    <Select.Item value="Female">Female</Select.Item>
			  </Select.Content>
			</Select.Root>
		</div>
	</div>
	<div class="w-full">
		<Button onclick={openDialog}>Save</Button>
	</div>
</div>

<Dialog.Root bind:open={isDialogOpen}>
  <Dialog.Content interactOutsideBehavior="ignore" escapeKeydownBehavior="ignore">
    <Dialog.Header>
      <Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
      <Dialog.Description>
        This action cannot be undone. This will permanently delete your account
        and remove your data from our servers.
      </Dialog.Description>
    </Dialog.Header>
  </Dialog.Content>
</Dialog.Root>