import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import './App.css'

type Stone = 0 | 1 | 2 // 0=空, 1=黒, 2=白
type Territory = 0 | 1 | 2 // 0=なし, 1=黒地, 2=白地

type BoardState = {
  size: number
  seed: number
  stones: ArrayLike<number>
  territory: ArrayLike<number>
}

type StoneMove = {
  color: number // 1=黒, 2=白
  from: [number, number] // [x, y]
  to: [number, number] // [x, y]
}

type LevelResp = {
  board: BoardState
  moves: StoneMove[]
}

// アニメーションタイミング定数
const ANIMATION_DELAY_MS = 1000 // 初期盤面表示時間
const ANIMATION_DURATION_MS = 900 // アニメーション完了までの時間

function normalizeU8ArrayLike(x: ArrayLike<number>): number[] {
  return Array.from(x as ArrayLike<number>, (n) => Number(n))
}

function StoneCircles(props: { 
  stoneDots: Array<{ x: number; y: number; v: Stone; move?: StoneMove; animating: boolean }>
  animating: boolean 
}) {
  const { stoneDots, animating } = props
  const refs = useRef<Map<string, SVGCircleElement>>(new Map())
  const animatingRef = useRef(false)
  const stoneDotsRef = useRef(stoneDots)
  stoneDotsRef.current = stoneDots

  useEffect(() => {
    if (!animating || animatingRef.current) return
    animatingRef.current = true

    // アニメーション開始: 次のフレームで最終位置に設定
    const timeoutId = setTimeout(() => {
      for (const { move } of stoneDotsRef.current) {
        if (!move) continue
        const key = `${move.to[0]},${move.to[1]}`
        const el = refs.current.get(key)
        if (el) {
          requestAnimationFrame(() => {
            el.setAttribute('cx', String(move.to[0]))
            el.setAttribute('cy', String(move.to[1]))
          })
        }
      }
    }, 10)

    return () => {
      clearTimeout(timeoutId)
      animatingRef.current = false
    }
  }, [animating])

  return (
    <>
      {stoneDots.map(({ x, y, v, move, animating: isAnimating }) => {
        const key = move ? `${move.to[0]},${move.to[1]}` : `${x},${y}`
        
        return (
          <circle
            key={`s:${key}`}
            ref={(el) => {
              if (el) {
                refs.current.set(key, el)
              } else {
                refs.current.delete(key)
              }
            }}
            cx={x}
            cy={y}
            r={0.46}
            className={`${v === 1 ? 'goStoneBlack' : 'goStoneWhite'} ${isAnimating ? 'goStoneAnimating' : ''}`}
          />
        )
      })}
    </>
  )
}

function GoBoard19(props: { 
  board: BoardState | null
  beforeBoard?: BoardState | null
  moves?: StoneMove[]
  animating?: boolean
  animatingPending?: boolean
}) {
  const lines = Array.from({ length: 19 }, (_, i) => i) // 0..18
  const star = [3, 9, 15]
  const stars: Array<{ x: number; y: number }> = []
  for (const y of star) for (const x of star) stars.push({ x, y })

  const size = props.board?.size ?? 19
  const stones = props.board ? normalizeU8ArrayLike(props.board.stones) : null
  const territory = props.board ? normalizeU8ArrayLike(props.board.territory) : null
  const beforeStones = props.beforeBoard ? normalizeU8ArrayLike(props.beforeBoard.stones) : null
  const moves = props.moves ?? []
  const animating = props.animating ?? false
  const animatingPending = props.animatingPending ?? false

  const territoryDots = useMemo(() => {
    if (!territory || size !== 19) return []
    const out: Array<{ x: number; y: number; v: Territory }> = []
    for (let i = 0; i < territory.length; i++) {
      const v = territory[i] as Territory
      if (v === 0) continue
      out.push({ x: i % 19, y: Math.floor(i / 19), v })
    }
    return out
  }, [territory, size])

  // 移動情報をマップに変換
  // from座標をキーにしたマップ（アニメーション開始時の位置から検索）
  const moveMapByFrom = useMemo(() => {
    const map = new Map<string, StoneMove>()
    for (const move of moves) {
      const key = `${move.from[0]},${move.from[1]}`
      map.set(key, move)
    }
    return map
  }, [moves])
  
  // to座標をキーにしたマップ（整地後の位置から検索）
  const moveMapByTo = useMemo(() => {
    const map = new Map<string, StoneMove>()
    for (const move of moves) {
      const key = `${move.to[0]},${move.to[1]}`
      map.set(key, move)
    }
    return map
  }, [moves])

  const stoneDots = useMemo(() => {
    // アニメーション中またはアニメーション待機中は初期盤面（beforeStones）を使用、そうでなければ整地後の盤面（stones）を使用
    const sourceStones = ((animating || animatingPending) && beforeStones) ? beforeStones : stones
    if (!sourceStones || size !== 19) return []
    const out: Array<{ x: number; y: number; v: Stone; move?: StoneMove; animating: boolean }> = []
    for (let i = 0; i < sourceStones.length; i++) {
      const v = sourceStones[i] as Stone
      if (v === 0) continue
      const x = i % 19
      const y = Math.floor(i / 19)
      const key = `${x},${y}`
      // アニメーション中または待機中はfrom位置から検索、そうでなければto位置から検索
      const move = (animating || animatingPending) ? moveMapByFrom.get(key) : moveMapByTo.get(key)
      // アニメーション中で、この石が移動する場合、from位置から開始
      const isAnimating = animating && move !== undefined
      out.push({ 
        x: isAnimating ? move!.from[0] : x, 
        y: isAnimating ? move!.from[1] : y, 
        v, 
        move,
        animating: isAnimating 
      })
    }
    return out
  }, [stones, beforeStones, size, moveMapByFrom, moveMapByTo, animating, animatingPending])

  return (
    <div className="goBoardWrap" aria-label="19×19の碁盤">
      <svg className="goBoard" viewBox="-1 -1 20 20" role="img" aria-label="碁盤">
        <rect x="-1" y="-1" width="20" height="20" className="goBoardBg" />

        <defs>
          <clipPath id="boardClip">
            <rect x={0} y={0} width={18} height={18} />
          </clipPath>
        </defs>

        {/* territory (under grid/stone) */}
        <g clipPath="url(#boardClip)">
          {territoryDots.map(({ x, y, v }) => (
            <rect
              key={`t:${x},${y}`}
              x={x - 0.5}
              y={y - 0.5}
              width={1}
              height={1}
              className={v === 1 ? 'goTerritoryBlack' : 'goTerritoryWhite'}
            />
          ))}
        </g>

        {/* grid lines */}
        {lines.map((i) => (
          <g key={i}>
            <line x1={0} y1={i} x2={18} y2={i} className="goBoardLine" />
            <line x1={i} y1={0} x2={i} y2={18} className="goBoardLine" />
          </g>
        ))}

        {/* star points (hoshi) */}
        {stars.map(({ x, y }) => (
          <circle key={`${x},${y}`} cx={x} cy={y} r={0.18} className="goBoardStar" />
        ))}

        {/* border (keep under stones) */}
        <rect x="0" y="0" width="18" height="18" className="goBoardBorder" />

        {/* stones */}
        <StoneCircles stoneDots={stoneDots} animating={animating} />
      </svg>
    </div>
  )
}

function App() {
  const [status, setStatus] = useState<'idle' | 'loading' | 'error'>('idle')
  const [seed, setSeed] = useState(1)
  const [board, setBoard] = useState<BoardState | null>(null)
  const [leveledBoard, setLeveledBoard] = useState<BoardState | null>(null)
  const [moves, setMoves] = useState<StoneMove[]>([])
  const [animating, setAnimating] = useState(false)
  const [animationPending, setAnimationPending] = useState(false)

  const postJson = useCallback(async <T,>(path: string, body: unknown): Promise<T> => {
    const res = await fetch(path, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    })
    if (!res.ok) {
      const text = await res.text()
      throw new Error(`${res.status} ${res.statusText}: ${text}`)
    }
    return (await res.json()) as T
  }, [])

  const territoryCounts = useMemo(() => {
    if (!board) return null
    const t = Array.from(board.territory as ArrayLike<number>, (n) => Number(n))
    let black = 0
    let white = 0
    for (const v of t) {
      if (v === 1) black++
      else if (v === 2) white++
    }
    return { black, white }
  }, [board])

  const generateBoard = useCallback(() => {
    if (status === 'loading') return
    const next = seed + 1
    setStatus('loading')
    ;(async () => {
      try {
        const v = await postJson<BoardState>('/api/board/generate', { seed: next })
        setSeed(next)
        setBoard(v)
        setLeveledBoard(null)
        setMoves([])
        setAnimating(false)
        setAnimationPending(false)
        setStatus('idle')
      } catch (e) {
        console.error('generate failed', e)
        setStatus('error')
      }
    })()
  }, [postJson, seed, status])

  const doLevel = useCallback(() => {
    if (!board) return
    if (status === 'loading') return
    setStatus('loading')
    setAnimating(false)
    setAnimationPending(false)
    ;(async () => {
      try {
        const resp = await postJson<LevelResp>('/api/board/level', { board })
        setLeveledBoard(resp.board)
        setMoves(resp.moves)
        setStatus('idle')
      } catch (e) {
        console.error('level failed', e)
        setStatus('error')
      }
    })()
  }, [board, postJson, status])

  const startAnimation = useCallback(() => {
    if (!leveledBoard || moves.length === 0) return
    // まず初期盤面を表示（アニメーション開始前の状態）
    setAnimationPending(true)
    // 待機時間後にアニメーションを開始
    setTimeout(() => {
      setAnimationPending(false)
      setAnimating(true)
      // アニメーション完了後
      setTimeout(() => {
        setAnimating(false)
      }, ANIMATION_DURATION_MS)
    }, ANIMATION_DELAY_MS)
  }, [leveledBoard, moves])

  useEffect(() => {
    // 初回はseed=1で生成（バックエンドから取得）
    let cancelled = false
    setStatus('loading')
    ;(async () => {
      try {
        const v = await postJson<BoardState>('/api/board/generate', { seed })
        if (cancelled) return
        setBoard(v)
        setLeveledBoard(null)
        setMoves([])
        setAnimating(false)
        setAnimationPending(false)
        setStatus('idle')
      } catch (e) {
        if (cancelled) return
        console.error('initial generate failed', e)
        setStatus('error')
      }
    })()
    return () => {
      cancelled = true
    }
  }, [postJson, seed])

  return (
    <>
      <h1>最適整地</h1>

      <div className="goControls">
        <button onClick={generateBoard} disabled={status === 'loading'}>
          盤面生成
        </button>
        <button onClick={doLevel} disabled={status === 'loading' || !board}>
          整地
        </button>
        <button 
          onClick={startAnimation} 
          disabled={status === 'loading' || !leveledBoard || moves.length === 0 || animating || animationPending}
        >
          再生
        </button>
        {territoryCounts && (
          <span className="goCounts">
            黒地: {territoryCounts.black} / 白地: {territoryCounts.white}
          </span>
        )}
      </div>
      <div className="goBoards">
        <div className="goBoardPane">
          <div className="goPaneTitle">生成盤面</div>
          <GoBoard19 board={board} />
        </div>
        <div className="goBoardPane">
          <div className="goPaneTitle">整地後</div>
          {leveledBoard ? (
            <GoBoard19 
              board={leveledBoard} 
              beforeBoard={board}
              moves={moves} 
              animating={animating}
              animatingPending={animationPending}
            />
          ) : (
            <p className="goHint">未整地です（整地ボタンを押してください）</p>
          )}
        </div>
      </div>
    </>
  )
}

export default App
