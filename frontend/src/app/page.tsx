export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm">
        <h1 className="text-4xl font-bold mb-8">APEIRON-X</h1>
        <p className="text-xl mb-4">
          HPM Mathematical Universe Axiom Verifier
        </p>
        <p className="text-lg mb-8">
          Dynamic Logic–Topology–Tensor Evolution System
        </p>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <a
            href="/hpm-dl-editor"
            className="p-6 border border-gray-300 rounded-lg hover:bg-gray-100"
          >
            <h2 className="text-xl font-semibold mb-2">HPM-DL Editor</h2>
            <p>Edit and compile HPM-DL source files</p>
          </a>
          
          <a
            href="/axiom-graph-editor"
            className="p-6 border border-gray-300 rounded-lg hover:bg-gray-100"
          >
            <h2 className="text-xl font-semibold mb-2">Axiom Graph Editor</h2>
            <p>Visualize and edit axiom graphs</p>
          </a>
          
          <a
            href="/certificate-viewer"
            className="p-6 border border-gray-300 rounded-lg hover:bg-gray-100"
          >
            <h2 className="text-xl font-semibold mb-2">Certificate Viewer</h2>
            <p>View existence certificates and proofs</p>
          </a>
        </div>
      </div>
    </main>
  )
}

