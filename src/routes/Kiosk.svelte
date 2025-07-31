<script lang="ts">
  import { menu } from '../data/menu';
  import { orders } from '../lib/orders';

  const ws = new WebSocket("ws://127.0.0.1:9001");

  ws.onmessage = (event) => {
    const updatedOrders = JSON.parse(event.data);
    orders.set(updatedOrders); // Update the store with new data
  };

  type MenuItem = {
    name: string;
    price: number;
    // add other properties if needed
  };

  let cart: MenuItem[] = [];

  function addToCart(item: MenuItem) {
    cart = [...cart, item];
  }

  function placeOrder() {
    orders.update((o) => {
      const newOrders = [
        ...o,
        { id: Date.now(), items: cart, status: "new" },
      ];
      ws.send(JSON.stringify(newOrders)); // Send updates to WebSocket server
      return newOrders;
    });
    cart = [];
  }
</script>

<h2>Menu</h2>
<ul>
  {#each menu as item}
    <li>
      {item.name} - ${item.price}
      <button on:click={() => addToCart(item)}>Add</button>
    </li>
  {/each}
</ul>

<h2>Cart</h2>
<ul>
  {#each cart as item}
    <li>{item.name}</li>
  {/each}
</ul>

<button on:click={placeOrder} disabled={cart.length === 0}>Place Order</button>

<h2>All Orders</h2>
<ul>
  {#each $orders as order}
    <li>Order #{order.id}: {order.status}</li>
  {/each}
</ul>