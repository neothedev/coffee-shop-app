<script lang="ts">
  import { orders } from '../lib/orders';

  const ws = new WebSocket("ws://127.0.0.1:9001");

  ws.onmessage = (event) => {
    const updatedOrders = JSON.parse(event.data);
    orders.set(updatedOrders); // Update the store with new data
  };

  function updateStatus(id: number, status: string) {
    orders.update((os) => {
      const updatedOrders = os.map((order) =>
        order.id === id ? { ...order, status } : order
      );
      ws.send(JSON.stringify(updatedOrders)); // Send updates to WebSocket server
      return updatedOrders;
    });
  }
</script>

<h1>Kitchen Orders</h1>
<ul>
  {#each $orders as order (order.id)}
    <li>
      <strong>Order #{order.id}</strong> - Status: {order.status}
      <ul>
        {#each order.items as item (item.name)}
          <li>{item.name} x {item.qty}</li>
        {/each}
      </ul>
      {#if order.status === 'new'}
        <button on:click={() => updateStatus(order.id, 'preparing')}>
          Mark Preparing
        </button>
      {:else if order.status === 'preparing'}
        <button on:click={() => updateStatus(order.id, 'ready')}>
          Mark Ready
        </button>
      {/if}
    </li>
  {/each}
</ul>