import Navbar from '@/components/Navbar'
import '@/styles/globals.css'

export default function App({ Component, pageProps }) {
  return (
    <>
      <span data-tauri-drag-region className='absolute w-screen h-7'></span>
      <Navbar />
      <Component {...pageProps} />
    </>
  )
}