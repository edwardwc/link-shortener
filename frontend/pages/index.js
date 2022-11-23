import Link from 'next/link';

export default function Home() {
  return (
          <div className="hero bg-base-200 min-h-screen">
              <div className="hero-content flex-col lg:flex-row-reverse">
                  <div className="text-center lg:text-left">
                      <h1 className="text-5xl font-bold">swath.cc: a link shortener</h1>
                      <p className="py-6">A high-performance and reliable link shortener, made in <Link href={"https://rust-lang.org"} className={"underline"}>Rust</Link>.
                      Uses <Link href={"https://sled.rs"} className={"underline"}>Sled</Link> and <Link href={"https://salvo.rs"} className={"underline"}>Salvo</Link>.
                      Check out the <Link href={"https://github.com/edwardwc/link-shortener"} className={"underline"}>code</Link>!</p>
                      <p className={"py-6 italic"}>Built on <Link href={"https://diamondcdn.com"} className={"underline"}>DiamondCDN</Link></p>
                  </div>
                  <div className="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                      <form onSubmit={shortenLinkForm} className="card-body">
                          <div className="form-control">
                              <label className="label">
                                  <span className="label-text">Your long link</span>
                              </label>
                              <input id="domain" required={true} type="text" placeholder="Those Google Doc links are pretty long" className="input input-bordered" />
                          </div>
                          <div className="form-control mt-6">
                              <button className="btn btn-primary">Shorten it!</button>
                          </div>
                      </form>
                      <footer className="footer footer-center p-4 bg-base-300 text-base-content">
                          <div>
                              <p>Edward C © 2024</p>
                          </div>
                      </footer>
                  </div>
              </div>
          </div>
  )
}
async function shortenLinkForm() {
    event.preventDefault()
    const result = await fetch('https://api.swath.cc/add-shortener', {
        body: JSON.stringify({
            domain: event.target.domain.value
        }),
        headers: {
            'Content-Type': 'application/json',
            'mode': 'no-cors'
        },
        method: 'POST',
    });
    alert(result);
}