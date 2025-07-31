<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  import Router from 'svelte-spa-router';
  import Kiosk from './routes/Kiosk.svelte';
  import Kitchen from './routes/Kitchen.svelte';
  import Admin from './routes/Admin.svelte';

  const routes = {
    '/kiosk': Kiosk,
    '/kitchen': Kitchen,
    '/admin': Admin,
    '*': Kiosk
  };
</script>

<nav>
  <a href="#/kiosk">Kiosk</a>
  <a href="#/kitchen">Kitchen</a>
  <a href="#/admin">Admin</a>
</nav>

<Router {routes} />


