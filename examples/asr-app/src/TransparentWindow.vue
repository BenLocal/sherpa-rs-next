<template>
  <div class="lyrics-panel">
    <div class="lyrics-container" data-tauri-drag-region>
      <div class="lyrics-scroll" data-tauri-drag-region>
        <div
          class="lyrics-line"
          v-for="(line, index) in lyrics"
          :key="index"
          :class="{
            active: index === currentLine,
            prev: index === currentLine - 1,
            next: index === currentLine + 1,
          }"
          data-tauri-drag-region
        >
          {{ line }}
        </div>
      </div>
    </div>
    <div class="close-btn-wrapper" @mousedown.stop @click.stop @mouseup.stop>
      <button
        class="close-btn"
        @mousedown="handleCloseMouseDown"
        @click="closeWindow"
        @mouseup.stop
      >
        ×
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

const currentLine = ref(2);
const lyrics = ref([
  "夜空中最亮的星",
  "能否听清",
  "那仰望的人",
  "心底的孤独和叹息",
  "夜空中最亮的星",
  "能否记起",
  "曾与我同行",
  "消失在风里的身影",
]);

async function handleCloseMouseDown(event: MouseEvent) {
  event.stopPropagation();
  event.preventDefault();
  try {
    const window = getCurrentWindow();
    await window.close();
  } catch (error) {
    console.error("关闭窗口失败:", error);
  }
}

async function closeWindow(event?: MouseEvent) {
  if (event) {
    event.stopPropagation();
    event.preventDefault();
  }
  try {
    const window = getCurrentWindow();
    await window.close();
  } catch (error) {
    console.error("关闭窗口失败:", error);
  }
}

setInterval(() => {
  currentLine.value = (currentLine.value + 1) % lyrics.value.length;
}, 4000);

onMounted(() => {
  // 组件已挂载
});
</script>

<style scoped>
.lyrics-panel {
  width: 100%;
  height: 100vh;
  background: linear-gradient(
    135deg,
    rgba(25, 25, 45, 0.75) 0%,
    rgba(15, 15, 35, 0.85) 100%
  );
  backdrop-filter: blur(25px) saturate(180%);
  -webkit-backdrop-filter: blur(25px) saturate(180%);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  color: white;
  font-family: "PingFang SC", "Microsoft YaHei", "Helvetica Neue", Arial,
    sans-serif;
  user-select: none;
  -webkit-user-select: none;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

.lyrics-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 60px;
  height: 100%;
  overflow: hidden;
  position: relative;
}

.lyrics-scroll {
  display: flex;
  align-items: center;
  gap: 30px;
  white-space: nowrap;
  transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.lyrics-line {
  font-size: 28px;
  font-weight: 400;
  opacity: 0.25;
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  white-space: nowrap;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  letter-spacing: 1px;
}

.lyrics-line.prev {
  opacity: 0.5;
  font-size: 24px;
  transform: translateX(-20px) scale(0.9);
}

.lyrics-line.active {
  opacity: 1;
  font-size: 38px;
  font-weight: 500;
  text-shadow: 0 0 15px rgba(255, 255, 255, 0.6),
    0 0 30px rgba(120, 180, 255, 0.4), 0 2px 8px rgba(0, 0, 0, 0.5);
  transform: scale(1.08);
  color: #ffffff;
  position: relative;
  letter-spacing: 2px;
}

.lyrics-line.active::before {
  content: "";
  position: absolute;
  left: -20px;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 70%;
  background: linear-gradient(
    180deg,
    rgba(120, 180, 255, 0.9) 0%,
    rgba(150, 200, 255, 0.7) 50%,
    rgba(120, 180, 255, 0.9) 100%
  );
  border-radius: 2px;
  box-shadow: 0 0 8px rgba(120, 180, 255, 0.8),
    0 0 15px rgba(120, 180, 255, 0.4);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
    transform: translateY(-50%) scaleY(1);
  }
  50% {
    opacity: 0.7;
    transform: translateY(-50%) scaleY(0.95);
  }
}

.lyrics-line.next {
  opacity: 0.5;
  font-size: 24px;
  transform: translateX(20px) scale(0.9);
}

.close-btn-wrapper {
  position: absolute;
  top: 8px;
  right: 12px;
  z-index: 1000;
  /* 容器可以接收事件 */
  pointer-events: auto;
  /* 确保不在拖拽区域内 */
  isolation: isolate;
}

.close-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: white;
  font-size: 20px;
  line-height: 1;
  cursor: pointer;
  padding: 4px 8px;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.2s;
  opacity: 0.6;
  pointer-events: auto;
}

.close-btn:hover {
  background: rgba(255, 100, 100, 0.3);
  opacity: 1;
  transform: scale(1.1);
}
</style>
