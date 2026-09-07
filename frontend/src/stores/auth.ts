import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token') || 'demo-token')
  const user = ref({
    id: '00000000-0000-0000-0000-000000000002',
    email: 'admin@omnirecon.local',
    fullName: 'System Administrator',
    role: 'ADMIN',
  })

  function logout() {
    token.value = null
    localStorage.removeItem('token')
  }

  return {
    token,
    user,
    logout,
  }
})
