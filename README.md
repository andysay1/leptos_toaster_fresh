![Example](assets/example.gif)

# leptos_toaster

A Toaster component for Leptos heavily inspired by [sonner](https://sonner.emilkowal.ski/)

## SSR
If using SSR don't forget to set the features in your own Project correctly
```toml
[features]
ssr = ["leptos_toaster/ssr"]
hydrate = ["leptos_toaster/hydrate"]

```




## Usage
Somewhere, probably near the top of your component tree, add the Toaster component
```rust
view! {
	<Toaster
	    position=toaster::ToasterPosition::BottomCenter
	>
		// ...
	</Toaster>
}
```
and then whenever you need a toast, do

```rust
let toast_context = expect_context::<Toasts>();

let create_toast = move || {
	let toast_id = ToastId::new();
	toast_context.toast(
		// This uses the built in toast component that requires the `builtin_toast` feature.
		// You can use your own components here
		move || view! {
			<Toast
				toast_id
				variant=ToastVariant::Info
				title=|| view! {"My toast"}
			/>
		},
		Some(toast_id),
		None // options
	);
}
```

## Styling with Tailwind

`Toaster` accepts `class` for the toaster list and `toast_container_class` for each toast container.
The built-in `Toast` keeps its default classes and accepts additional Tailwind classes.
For Tailwind 4, pass literal class strings from your app code so Tailwind can discover them.

```rust
view! {
	<Toast
		toast_id
		variant=ToastVariant::Success
		title=|| view! { "Saved" }
		class="border-emerald-200 bg-emerald-50 text-emerald-950 shadow-lg"
		close_button_class="border-emerald-200 bg-emerald-100 hover:bg-emerald-200"
		icon_class="text-emerald-600"
		title_class="font-semibold"
		description_class="text-emerald-800"
	/>
}
```

If you want Tailwind to control all toast styling, disable the built-in toast stylesheet.
The structural toaster CSS is still mounted by `<Toaster />`.

```rust
view! {
	<Toast
		toast_id
		unstyled=true
		title=|| view! { "Custom Tailwind toast" }
		class="flex w-96 items-center gap-3 rounded-xl border border-zinc-800 bg-zinc-950 p-4 text-sm text-white shadow-xl"
		close_button_class="absolute right-2 top-2 rounded-md p-1 text-zinc-400 hover:bg-zinc-800 hover:text-white"
		title_class="font-medium"
		description_class="text-zinc-400"
	/>
}
```
