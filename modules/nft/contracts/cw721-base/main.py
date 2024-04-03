import os, re

def extract_raw_log():
    with open('log.file', 'r') as file:
        # 파일 내용을 모두 읽어들임
        content = file.read()
        # raw_log: 다음부터 timestamp: 전까지의 내용 추출
        raw_log_start = content.find('raw_log:') + len('raw_log:')
        timestamp_end = content.find('timestamp:')
        raw_log_content = content[raw_log_start:timestamp_end].strip()
    return raw_log_content

# 명령어의 출력을 파일로 저장
def process_line(line, shell_script):
    result_file = 'log.file'

    sys_command = 'bash ' + shell_script + ' > ' + result_file
    os.system(sys_command)

    f = open(result_file, 'r')
    
# 파일을 한 줄씩 읽어들여서 처리하는 함수
def process_commands(command_file):
    with open(command_file, 'r') as file:
        for line in file:
            process_line(line.strip(), command_file)
            
            match = re.search(r'"token_id": "([^"]+)"', line)
            if match:
                token_id = match.group(1)
                result = token_id + " : " + extract_raw_log()
                with open("result.txt", "a") as file:
                    file.write(result + "\n")
                print(token_id + " is done.")
                
            else:
                with open("result.txt", "a") as file:
                    file.write(token_id + " : error" + "\n")
                print("error with " + token_id)
            


command_file = 'commands.txt' 
process_commands(command_file)