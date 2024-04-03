import os

shell_script = 'test.sh'
result_file = 'log.file'

sys_command = 'bash ' + shell_script + ' > ' + result_file
os.system(sys_command)

f = open(result_file, 'r')

def extract_raw_log(file_path):
    with open(file_path, 'r') as file:
        # 파일 내용을 모두 읽어들임
        content = file.read()
        # raw_log: 다음부터 timestamp: 전까지의 내용 추출
        raw_log_start = content.find('raw_log:') + len('raw_log:')
        timestamp_end = content.find('timestamp:')
        raw_log_content = content[raw_log_start:timestamp_end].strip()
    return raw_log_content

# 파일 경로
file_path = 'log.file'  # 파일 경로를 실제 파일의 경로로 변경하세요.

# raw_log의 뒷 문구 추출
raw_log_tail = extract_raw_log(file_path)
print(raw_log_tail)