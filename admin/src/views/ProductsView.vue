<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Product {
  id: number
  name: string
  price: number
  description: string | null
}

const products = ref<Product[]>([])

onMounted(async () => {
  try {
    const response = await fetch('http://localhost:3000/api/products')
    if (response.ok) {
      products.value = await response.json()
    } else {
      console.error('Failed to fetch products')
    }
  } catch (error) {
    console.error('An error occurred while fetching products:', error)
  }
})
</script>

<template>
  <div class="container mx-auto p-8">
    <h1 class="text-3xl font-bold mb-6 text-center">Our Products</h1>
    <div v-if="products.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      <div v-for="product in products" :key="product.id" class="bg-white rounded-lg shadow-md overflow-hidden">
        <div class="p-6">
          <h2 class="text-xl font-bold mb-2">{{ product.name }}</h2>
          <p class="text-gray-700 mb-4">{{ product.description }}</p>
          <p class="text-2xl font-semibold text-gray-900">${{ (product.price / 100).toFixed(2) }}</p>
           <button class="mt-4 w-full bg-gray-800 text-white py-2 rounded hover:bg-gray-900 transition-colors">
            Buy Now
          </button>
        </div>
      </div>
    </div>
    <div v-else class="text-center text-gray-500 mt-10">
      <p>No products available at the moment.</p>
    </div>
  </div>
</template>

<style>
.products {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
}
ul {
  list-style: none;
}
</style> 