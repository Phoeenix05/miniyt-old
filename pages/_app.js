import Navbar from '@/components/Navbar'
import '@/styles/globals.css'

export default function App({ Component, pageProps }) {
  return (
    <>
      <span data-tauri-drag-region className='absolute top-0 left-0 w-screen h-7'></span>
      <Navbar />
      <Component {...pageProps} />
    </>
  )
}