<script lang="ts">
  import * as Select from "components/ui/select";
  import { COLORS } from "../../constants/colors";

  // Svelte 5 bindable prop
  let { data = $bindable() } = $props<{ data: string | null }>();

  // Use a derived value to find the active color object for the trigger UI
  const selectedColor = $derived(
    COLORS.find((c) => c.value === data) ?? COLORS[0],
  );
</script>

<Select.Root
  type="single"
  value={data ?? COLORS[0].value}
  onValueChange={(v) => {
    data = v;
  }}
  name="color"
>
  <Select.Trigger class="w-full">
    <div class="flex items-center gap-2">
      <div
        class="h-4 w-4 rounded-full border border-white/20"
        style:background-color={selectedColor.value}
      ></div>
      <span>{selectedColor.label}</span>
    </div>
  </Select.Trigger>

  <Select.Content>
    {#each COLORS as color}
      <Select.Item value={color.value} label={color.label}>
        <div class="flex items-center gap-2">
          <div
            class="h-4 w-4 rounded-full border border-black/10 dark:border-white/20"
            style:background-color={color.value}
          ></div>
          <span>{color.label}</span>
        </div>
      </Select.Item>
    {/each}
  </Select.Content>
</Select.Root>
