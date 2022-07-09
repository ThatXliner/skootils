<script>
	import '../app.css';

	import Notification from '$lib/Notification.svelte';
	import Notifications from 'svelte-notifications';
</script>

<Notifications item={Notification}><slot /></Notifications>
